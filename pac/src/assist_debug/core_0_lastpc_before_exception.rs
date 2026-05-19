#[doc = "Register `CORE_0_LASTPC_BEFORE_EXCEPTION` reader"]
pub type R = crate::R<Core0LastpcBeforeExceptionSpec>;
#[doc = "Field `CORE_0_LASTPC_BEFORE_EXC` reader - cpu's lastpc before exception"]
pub type Core0LastpcBeforeExcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cpu's lastpc before exception"]
    #[inline(always)]
    pub fn core_0_lastpc_before_exc(&self) -> Core0LastpcBeforeExcR {
        Core0LastpcBeforeExcR::new(self.bits)
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_lastpc_before_exception::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0LastpcBeforeExceptionSpec;
impl crate::RegisterSpec for Core0LastpcBeforeExceptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_lastpc_before_exception::R`](R) reader structure"]
impl crate::Readable for Core0LastpcBeforeExceptionSpec {}
#[doc = "`reset()` method sets CORE_0_LASTPC_BEFORE_EXCEPTION to value 0"]
impl crate::Resettable for Core0LastpcBeforeExceptionSpec {}
