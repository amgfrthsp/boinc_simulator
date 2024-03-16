use serde::Serialize;

use dslab_compute::multicore::CoresDependency;

#[derive(Serialize, Debug, Clone)]
pub struct JobRequest {
    pub id: u64,
    pub flops: f64,
    pub memory: u64,
    pub min_cores: u32,
    pub max_cores: u32,
    pub cores_dependency: CoresDependency,
    pub input_size: u64,
    pub output_size: u64,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ResultState {
    Inactive,
    Unsent,
    InProgress,
    Over,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ResultOutcome {
    Success,
    CouldntSend,
    ClientError,
    NoReply,
    DidntNeed,
    ValidateError,
    ClientDetached,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ValidateState {
    Init,
    Valid,
    Invalid,
    NoCheck,
    Error,
    Inconclusive,
    TooLate,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AssimilateState {
    Init,
    Ready,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FileDeleteState {
    Init,
    Ready,
    Done,
}

#[derive(Debug, Clone)]
pub struct WorkunitInfo {
    pub(crate) id: u64,
    pub(crate) req: JobRequest,
    pub(crate) result_ids: Vec<u64>,
    pub(crate) transition_time: f64,
    pub(crate) delay_bound: f64,
    pub(crate) min_quorum: u64,
    pub(crate) target_nresults: u64,
    pub(crate) need_validate: bool,
    pub(crate) file_delete_state: FileDeleteState,
    pub(crate) canonical_resultid: Option<u64>,
    pub(crate) assimilate_state: AssimilateState,
}

#[derive(Debug, Clone)]
pub struct ResultInfo {
    pub(crate) id: u64,
    pub(crate) workunit_id: u64,
    pub(crate) report_deadline: f64,
    pub(crate) server_state: ResultState,
    pub(crate) outcome: Option<ResultOutcome>,
    pub(crate) validate_state: Option<ValidateState>,
}
