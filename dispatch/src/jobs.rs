use std::collections::BTreeSet;
use std::net::IpAddr;
use std::time::{Duration, SystemTime};

use crate::github::Asset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    Unassigned,
    Assigned(IpAddr),
    Downloading(IpAddr),
    Booting(IpAddr),
    Reported(IpAddr),
    Finished(IpAddr),
    Failed(IpAddr),
}

impl State {
    pub const fn ip(&self) -> Option<IpAddr> {
        match self {
            Self::Unassigned => None,

            Self::Assigned(ip)
            | Self::Downloading(ip)
            | Self::Booting(ip)
            | Self::Reported(ip)
            | Self::Finished(ip)
            | Self::Failed(ip) => Some(*ip),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Job {
    pub asset: Asset,
    pub state: State,
    pub seen: Option<SystemTime>,
}

impl Job {
    fn elapsed(&self) -> Duration {
        self.seen
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .elapsed()
            .unwrap_or_default()
    }
}

pub struct Jobs(Vec<Job>);

impl From<BTreeSet<Asset>> for Jobs {
    fn from(assets: BTreeSet<Asset>) -> Self {
        Self(
            assets
                .into_iter()
                .map(|asset| Job {
                    asset,
                    state: State::Unassigned,
                    seen: None,
                })
                .collect(),
        )
    }
}

impl Jobs {
    const TIMEOUT: Duration = Duration::from_mins(5);

    pub fn iter(&self) -> impl Iterator<Item = &Job> {
        self.0.iter()
    }

    pub fn assign(&mut self, ip: IpAddr) -> Option<Asset> {
        // First, try to find a job that is already assigned to this IP.
        for job in &mut self.0 {
            match job.state {
                State::Assigned(addr) if addr == ip => {
                    job.state = State::Assigned(ip);
                    job.seen = Some(SystemTime::now());
                    return Some(job.asset.clone());
                }

                _ => {}
            }
        }

        // Next, try to find an unassigned or expired job.
        for job in &mut self.0 {
            match job.state {
                State::Unassigned => {
                    job.state = State::Assigned(ip);
                    job.seen = Some(SystemTime::now());
                    return Some(job.asset.clone());
                }

                State::Assigned(..) | State::Downloading(..) if job.elapsed() > Self::TIMEOUT => {
                    job.state = State::Assigned(ip);
                    job.seen = Some(SystemTime::now());
                    return Some(job.asset.clone());
                }

                _ => {}
            }
        }

        None
    }

    pub fn downloading(&mut self, ip: IpAddr) -> Option<&Asset> {
        for job in &mut self.0 {
            match job.state {
                State::Assigned(addr) if addr == ip => {
                    job.state = State::Downloading(ip);
                    job.seen = Some(SystemTime::now());
                    return Some(&job.asset);
                }
                _ => {}
            }
        }

        None
    }

    pub fn booting(&mut self, ip: IpAddr) -> bool {
        for job in &mut self.0 {
            match job.state {
                State::Downloading(addr) if addr == ip => {
                    job.state = State::Booting(ip);
                    job.seen = Some(SystemTime::now());
                    return true;
                }
                _ => {}
            }
        }

        false
    }

    pub fn report(&mut self, ip: IpAddr) -> bool {
        for job in &mut self.0 {
            match job.state {
                State::Booting(addr) if addr == ip => {
                    job.state = State::Reported(ip);
                    job.seen = Some(SystemTime::now());
                    return true;
                }
                _ => {}
            }
        }

        false
    }

    pub fn finish(&mut self, ip: IpAddr) -> bool {
        for job in &mut self.0 {
            job.state = match job.state {
                State::Assigned(addr) if addr == ip => State::Failed(ip),
                State::Booting(addr) if addr == ip => State::Failed(ip),
                State::Reported(addr) if addr == ip => State::Finished(ip),
                _ => continue,
            };

            job.seen = Some(SystemTime::now());
            return true;
        }

        false
    }
}
