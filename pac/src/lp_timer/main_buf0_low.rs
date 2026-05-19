#[doc = "Register `MAIN_BUF0_LOW` reader"]
pub type R = crate::R<MainBuf0LowSpec>;
#[doc = "Field `MAIN_TIMER_BUF0_LOW` reader - need_des"]
pub type MainTimerBuf0LowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_buf0_low(&self) -> MainTimerBuf0LowR {
        MainTimerBuf0LowR::new(self.bits)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_buf0_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainBuf0LowSpec;
impl crate::RegisterSpec for MainBuf0LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`main_buf0_low::R`](R) reader structure"]
impl crate::Readable for MainBuf0LowSpec {}
#[doc = "`reset()` method sets MAIN_BUF0_LOW to value 0"]
impl crate::Resettable for MainBuf0LowSpec {}
