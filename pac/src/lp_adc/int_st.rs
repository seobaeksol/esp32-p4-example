#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `COCPU_SARADC1_INT_ST` reader - ADC1 Conversion is done, int status."]
pub type CocpuSaradc1IntStR = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_ST` reader - ADC2 Conversion is done, int status."]
pub type CocpuSaradc2IntStR = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_ST` reader - An errro occurs from ADC1, int status."]
pub type CocpuSaradc1ErrorIntStR = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_ST` reader - An errro occurs from ADC2, int status."]
pub type CocpuSaradc2ErrorIntStR = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_ST` reader - A wakeup event is triggered from ADC1, int status."]
pub type CocpuSaradc1WakeIntStR = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_ST` reader - A wakeup event is triggered from ADC2, int status."]
pub type CocpuSaradc2WakeIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC1 Conversion is done, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_st(&self) -> CocpuSaradc1IntStR {
        CocpuSaradc1IntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_st(&self) -> CocpuSaradc2IntStR {
        CocpuSaradc2IntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_st(&self) -> CocpuSaradc1ErrorIntStR {
        CocpuSaradc1ErrorIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_st(&self) -> CocpuSaradc2ErrorIntStR {
        CocpuSaradc2ErrorIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_st(&self) -> CocpuSaradc1WakeIntStR {
        CocpuSaradc1WakeIntStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_st(&self) -> CocpuSaradc2WakeIntStR {
        CocpuSaradc2WakeIntStR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt status registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
