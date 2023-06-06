use tun::AsyncDevice;

use crate::Result;

pub struct Tun {
    tun: AsyncDevice,
}

impl Tun {
    pub fn new(config: Config) -> Result<Self> {
        let tun = {
            let mut config = tun::configure();

            // config.address()
        };

        Ok(Self { tun })
    }
}
