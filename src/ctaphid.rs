#[cfg(feature = "ctaphid")]
use crate::Authenticator;
#[cfg(feature = "ctaphid")]
use iso7816::Status;
#[cfg(feature = "ctaphid")]
use trussed::client;

#[cfg(feature = "ctaphid")]
use ctaphid_dispatch::app::{self, Command as HidCommand, Message};
#[cfg(feature = "ctaphid")]
use ctaphid_dispatch::command::VendorCommand;

#[cfg(feature = "ctaphid")]
pub const OTP_CCID: VendorCommand = VendorCommand::H70;

#[cfg(feature = "ctaphid")]
impl<T> app::App for Authenticator<T>
where
    T: trussed::Client
        + client::HmacSha1
        + client::HmacSha256
        + client::Sha256
        + client::Chacha8Poly1305,
{
    fn commands(&self) -> &'static [HidCommand] {
        &[HidCommand::Vendor(OTP_CCID)]
    }

    fn call(
        &mut self,
        command: HidCommand,
        input_data: &Message,
        response: &mut Message,
    ) -> app::AppResult {
        const MAX_COMMAND_LENGTH: usize = 255;
        match command {
            HidCommand::Vendor(OTP_CCID) => {
                let arr: [u8; 2] = Status::Success.into();
                response.extend(arr);
                let ctap_to_iso7816_command =
                    iso7816::Command::<MAX_COMMAND_LENGTH>::try_from(input_data).map_err(|_e| {
                        response.clear();
                        debug_now!("ISO conversion error: {:?}", _e);
                        app::Error::InvalidLength
                    })?;
                self.respond(&ctap_to_iso7816_command, response)
                    .map_err(|e| {
                        debug_now!("OTP command execution error: {:?}", e);
                        let arr: [u8; 2] = e.into();
                        response.clear();
                        response.extend(arr);
                        app::Error::InvalidCommand
                    })
                    .ok();
            }
            _ => {
                return Err(app::Error::InvalidCommand);
            }
        }
        Ok(())
    }
}
