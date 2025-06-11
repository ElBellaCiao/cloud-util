use crate::InstanceId;
use aws_sdk_ec2::types::InstanceStateName;
use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Terminating,
    Terminated,
    Unknown,
}

impl From<&InstanceStateName> for InstanceState {
    fn from(aws_state: &InstanceStateName) -> Self {
        match aws_state {
            InstanceStateName::Pending => InstanceState::Starting,
            InstanceStateName::Running => InstanceState::Running,
            InstanceStateName::Stopping => InstanceState::Stopping,
            InstanceStateName::Stopped => InstanceState::Stopped,
            InstanceStateName::ShuttingDown => InstanceState::Terminating,
            InstanceStateName::Terminated => InstanceState::Terminated,
            _ => InstanceState::Unknown,
        }
    }
}

pub struct InstanceMetadata {
    pub instance_id: InstanceId,
    pub private_ip: IpAddr,
    pub status: InstanceState,
}
