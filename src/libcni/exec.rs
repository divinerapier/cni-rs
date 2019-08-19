use crate::libcni::result::Result;
use std::{collections::HashMap, io::Write};
use std::{
    io::Read,
    process::{Command, Stdio},
};

pub trait Exec {
    fn exec_plugins(
        &self,
        plugin_path: String,
        stdin_data: &[u8],
        environ: Vec<String>,
    ) -> super::Result<Vec<u8>>;
    fn find_in_path(&self, plugin: String, paths: Vec<String>) -> Result<String>;
    fn decode(&self, data: &[u8]) -> Result<()>;
}

pub struct RawExec<W: std::io::Write> {
    stderr: W,
}

impl<W> Exec for RawExec<W>
where
    W: std::io::Write,
{
    fn exec_plugins(
        &self,
        plugin_path: String,
        stdin_data: &[u8],
        environ: Vec<String>,
    ) -> Result<Vec<u8>> {
        let envs: HashMap<String, String> = environ
            .iter()
            .map(|key| key.split("=").collect::<Vec<&str>>())
            .map(|kv| (kv[0].to_string(), kv[1].to_string()))
            .collect();
        let mut plugin_cmd = std::process::Command::new(plugin_path)
            .envs(envs)
            .spawn()
            .unwrap();
        {
            let mut stdin = plugin_cmd.stdin.take().unwrap();
            stdin.write(stdin_data).unwrap();
        }

        let output = plugin_cmd.wait_with_output().unwrap();
        Ok(output.stdout)
    }

    fn find_in_path(&self, plugin: String, paths: Vec<String>) -> Result<String> {
        todo!()
    }

    fn decode(&self, data: &[u8]) -> Result<()> {
        todo!()
    }
}

struct BufferedStdin<'a> {
    buf: &'a [u8],
}

impl<'a> BufferedStdin<'a> {
    fn new(buf: &'a [u8]) -> BufferedStdin {
        BufferedStdin { buf }
    }
}

impl<'a> Into<Stdio> for BufferedStdin<'a> {
    fn into(self) -> Stdio {
        todo!()
    }
}
