#[doc = "Register `RD_WR_DIS` reader"]
pub type R = crate::R<RdWrDisSpec>;
#[doc = "Field `WR_DIS` reader - Represents whether programming of individual eFuse memory bit is disabled. For mapping between the bits of this field and the eFuse memory bits, please refer to Table \\ref{tab:efuse-block0-para} and Table \\ref{tab:efuse-block-1-10-para}.\\\\1: Disabled\\\\0: Enabled\\\\"]
pub type WrDisR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents whether programming of individual eFuse memory bit is disabled. For mapping between the bits of this field and the eFuse memory bits, please refer to Table \\ref{tab:efuse-block0-para} and Table \\ref{tab:efuse-block-1-10-para}.\\\\1: Disabled\\\\0: Enabled\\\\"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WrDisR {
        WrDisR::new(self.bits)
    }
}
#[doc = "Represents rd_wr_dis\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_wr_dis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdWrDisSpec;
impl crate::RegisterSpec for RdWrDisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_wr_dis::R`](R) reader structure"]
impl crate::Readable for RdWrDisSpec {}
#[doc = "`reset()` method sets RD_WR_DIS to value 0"]
impl crate::Resettable for RdWrDisSpec {}
