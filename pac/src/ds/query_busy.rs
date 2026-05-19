#[doc = "Register `QUERY_BUSY` reader"]
pub type R = crate::R<QueryBusySpec>;
#[doc = "Field `QUERY_BUSY` reader - Represents whether or not the DS module is idle.\\\\ 0: The DS module is idle\\\\ 1: The DS module is busy\\\\"]
pub type QueryBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not the DS module is idle.\\\\ 0: The DS module is idle\\\\ 1: The DS module is busy\\\\"]
    #[inline(always)]
    pub fn query_busy(&self) -> QueryBusyR {
        QueryBusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status of the DS module\n\nYou can [`read`](crate::Reg::read) this register and get [`query_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QueryBusySpec;
impl crate::RegisterSpec for QueryBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_busy::R`](R) reader structure"]
impl crate::Readable for QueryBusySpec {}
#[doc = "`reset()` method sets QUERY_BUSY to value 0"]
impl crate::Resettable for QueryBusySpec {}
