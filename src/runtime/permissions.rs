use std::path::Path;
use std::path::PathBuf;

use deno_path_util::normalize_path;
use deno_permissions::AllowRunDescriptorParseResult;
use deno_permissions::DenyRunDescriptor;
use deno_permissions::EnvDescriptor;
use deno_permissions::FfiDescriptor;
use deno_permissions::ImportDescriptor;
use deno_permissions::NetDescriptor;
use deno_permissions::PathQueryDescriptor;
use deno_permissions::PathResolveError;
use deno_permissions::ReadDescriptor;
use deno_permissions::RunDescriptorParseError;
use deno_permissions::RunQueryDescriptor;
use deno_permissions::SysDescriptor;
use deno_permissions::SysDescriptorParseError;
use deno_permissions::WriteDescriptor;

/**
 * This struct is required by Deno core
 * For now, I've created one with all permissions granted
 */

#[derive(Debug)]
pub struct RuntimePermissionDescriptorParser {
    current_dir: PathBuf,
}

impl RuntimePermissionDescriptorParser {
    pub fn new(current_dir: PathBuf) -> Self {
        Self { current_dir }
    }

    fn resolve_from_cwd(&self, path: &str) -> Result<PathBuf, PathResolveError> {
        if path.is_empty() {
            return Err(PathResolveError::EmptyPath);
        }
        let path = Path::new(path);
        if path.is_absolute() {
            Ok(normalize_path(path))
        } else {
            let cwd = &self.current_dir;
            Ok(normalize_path(cwd.join(path)))
        }
    }
}

impl deno_permissions::PermissionDescriptorParser for RuntimePermissionDescriptorParser {
    fn parse_read_descriptor(&self, text: &str) -> Result<ReadDescriptor, PathResolveError> {
        Ok(ReadDescriptor(self.resolve_from_cwd(text)?))
    }

    fn parse_write_descriptor(&self, text: &str) -> Result<WriteDescriptor, PathResolveError> {
        Ok(WriteDescriptor(self.resolve_from_cwd(text)?))
    }

    fn parse_net_descriptor(
        &self,
        text: &str,
    ) -> Result<NetDescriptor, deno_permissions::NetDescriptorParseError> {
        NetDescriptor::parse_for_list(text)
    }

    fn parse_import_descriptor(
        &self,
        text: &str,
    ) -> Result<ImportDescriptor, deno_permissions::NetDescriptorParseError> {
        ImportDescriptor::parse_for_list(text)
    }

    fn parse_env_descriptor(
        &self,
        text: &str,
    ) -> Result<EnvDescriptor, deno_permissions::EnvDescriptorParseError> {
        if text.is_empty() {
            Err(deno_permissions::EnvDescriptorParseError)
        } else {
            Ok(EnvDescriptor::new(text))
        }
    }

    fn parse_sys_descriptor(&self, text: &str) -> Result<SysDescriptor, SysDescriptorParseError> {
        if text.is_empty() {
            Err(SysDescriptorParseError::Empty)
        } else {
            Ok(SysDescriptor::parse(text.to_string())?)
        }
    }

    fn parse_allow_run_descriptor(
        &self,
        _text: &str,
    ) -> Result<AllowRunDescriptorParseResult, RunDescriptorParseError> {
        Err(RunDescriptorParseError::EmptyRunQuery)
    }

    fn parse_deny_run_descriptor(&self, text: &str) -> Result<DenyRunDescriptor, PathResolveError> {
        Ok(DenyRunDescriptor::parse(text, &self.current_dir))
    }

    fn parse_ffi_descriptor(&self, text: &str) -> Result<FfiDescriptor, PathResolveError> {
        Ok(FfiDescriptor(self.resolve_from_cwd(text)?))
    }

    // queries

    fn parse_path_query(&self, path: &str) -> Result<PathQueryDescriptor, PathResolveError> {
        Ok(PathQueryDescriptor {
            resolved: self.resolve_from_cwd(path)?,
            requested: path.to_string(),
        })
    }

    fn parse_net_query(
        &self,
        text: &str,
    ) -> Result<NetDescriptor, deno_permissions::NetDescriptorParseError> {
        NetDescriptor::parse_for_query(text)
    }

    fn parse_run_query(
        &self,
        _requested: &str,
    ) -> Result<RunQueryDescriptor, RunDescriptorParseError> {
        Err(RunDescriptorParseError::EmptyRunQuery)
    }
}
