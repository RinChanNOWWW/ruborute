use crate::config::Config;
use crate::data_source::DataSource;
use crate::{command::*, data_source, Result};
use prettytable::{cell, row, Cell, Row, Table};
use rustyline::{
    completion::Completer,
    error::ReadlineError,
    hint::{Hinter, HistoryHinter},
    Editor,
};
use rustyline_derive::{Helper, Highlighter, Validator};
use std::{collections::HashMap, rc::Rc, vec};

struct CmdCompleter {
    commands: Vec<String>,
}

impl CmdCompleter {
    fn new() -> Self {
        CmdCompleter {
            commands: Vec::new(),
        }
    }
    fn add_command(&mut self, cmd: String) {
        self.commands.push(cmd)
    }
}

impl Completer for CmdCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let res = self
            .commands
            .iter()
            .filter(|s| s.starts_with(line))
            .map(|s| s.clone() + " ")
            .collect::<Vec<String>>();
        Ok((0, res))
    }
}

#[derive(Helper, Highlighter, Validator)]
struct CmdlineHelper {
    cmd_completer: CmdCompleter,
    cmd_hinter: HistoryHinter,
}

impl CmdlineHelper {
    fn add_command(&mut self, cmd: String) {
        self.cmd_completer.add_command(cmd);
    }
}

impl Completer for CmdlineHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.cmd_completer.complete(line, pos, ctx)
    }
}

impl Hinter for CmdlineHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.cmd_hinter.hint(line, pos, ctx)
    }
}

pub struct Cmdline {
    help_table: Table,
    cmds: HashMap<String, Box<dyn Cmd>>,
    rl: Editor<CmdlineHelper>,
}

impl Cmdline {
    pub fn new(cfg: Config) -> Result<Self> {
        let rustyline_config = rustyline::Config::builder()
            .history_ignore_dups(true)
            .history_ignore_space(true)
            .completion_type(rustyline::CompletionType::List)
            .output_stream(rustyline::OutputStreamType::Stdout)
            .build();

        let mut helper = CmdlineHelper {
            cmd_completer: CmdCompleter::new(),
            cmd_hinter: HistoryHinter {},
        };
        let cmds: HashMap<String, Box<dyn Cmd>> = HashMap::new();
        let mut help_table = Table::new();
        help_table.add_row(row!["name", "usage", "description"]);
        help_table.add_row(row!["help", "help", "show the help information."]);
        helper.add_command(String::from("help"));
        let mut rl = Editor::with_config(rustyline_config);
        rl.set_helper(Some(helper));
        let mut cmdline = Cmdline {
            cmds,
            help_table,
            rl,
        };

        match data_source::AsphyxiaDataSource::open(cfg.asyphyxia) {
            // load from asyphyxia first
            Ok(s) => cmdline.add_commands(Rc::new(s)),
            // if load asyphyxia, load from bemaniutils server
            _ => cmdline.add_commands(Rc::new(data_source::BemaniutilsDataSource::open(
                cfg.bemaniutils,
            )?)),
        };

        Ok(cmdline)
    }

    /// add all supported commands
    fn add_commands<D: 'static + DataSource>(&mut self, ds: Rc<D>) {
        self.add_command(Box::new(CmdRecord::new(Rc::clone(&ds))));
        self.add_command(Box::new(CmdBest50::new(Rc::clone(&ds))));
        self.add_command(Box::new(CmdVolforce::new(Rc::clone(&ds))));
        self.add_command(Box::new(CmdCount::new(Rc::clone(&ds))));
    }

    fn add_command(&mut self, cmd: Box<dyn Cmd>) {
        self.help_table.add_row(Row::new(vec![
            Cell::new(cmd.name()),
            Cell::new(cmd.usage()),
            Cell::new(cmd.description()),
        ]));
        if let Some(helper) = self.rl.helper_mut() {
            helper.add_command(cmd.name().to_string());
        }
        self.cmds.insert(cmd.name().to_string(), cmd);
    }

    fn help(&self) {
        self.help_table.printstd();
    }

    pub fn run(&mut self) -> Result<()> {
        // run interactive cmdline
        loop {
            let readline = self.rl.readline(">> ");
            match readline {
                Ok(line) => {
                    self.rl.add_history_entry(line.as_str());
                    let cmds: Vec<String> = line
                        .trim()
                        .split(" ")
                        .filter(|&s| s.ne(""))
                        .map(|s| String::from(s))
                        .collect();
                    self.interact(cmds);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("<Keyboard Interrupted>");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Bye");
                    break;
                }
                Err(e) => {
                    println!("read command error: {}", &e);
                    break;
                }
            }
        }

        Ok(())
    }

    fn interact(&self, cmds: Vec<String>) {
        if cmds.len() == 0 {
            return;
        }
        if cmds[0].as_str() == "help" {
            self.help();
            return;
        }
        if let Some(cmd) = self.cmds.get(&cmds[0]) {
            if let Err(e) = cmd.do_cmd(&cmds[1..]) {
                println!("{}", e);
            }
        } else {
            println!("no such command");
        }
    }
}
