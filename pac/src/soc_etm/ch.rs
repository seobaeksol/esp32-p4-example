#[repr(C)]
#[doc = "Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID"]
#[doc(alias = "CH")]
pub struct Ch {
    evt_id: EvtId,
    task_id: TaskId,
}
impl Ch {
    #[doc = "0x00 - Channel0 event id register"]
    #[inline(always)]
    pub const fn evt_id(&self) -> &EvtId {
        &self.evt_id
    }
    #[doc = "0x04 - Channel0 task id register"]
    #[inline(always)]
    pub const fn task_id(&self) -> &TaskId {
        &self.task_id
    }
}
#[doc = "EVT_ID (rw) register accessor: Channel0 event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_id`] module"]
#[doc(alias = "EVT_ID")]
pub type EvtId = crate::Reg<evt_id::EvtIdSpec>;
#[doc = "Channel0 event id register"]
pub mod evt_id;
#[doc = "TASK_ID (rw) register accessor: Channel0 task id register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_id`] module"]
#[doc(alias = "TASK_ID")]
pub type TaskId = crate::Reg<task_id::TaskIdSpec>;
#[doc = "Channel0 task id register"]
pub mod task_id;
