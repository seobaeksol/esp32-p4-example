#[doc = "Register `DPC_CONF` reader"]
pub type R = crate::R<DpcConfSpec>;
#[doc = "Register `DPC_CONF` writer"]
pub type W = crate::W<DpcConfSpec>;
#[doc = "Field `DPC_THRESHOLD_L` reader - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DpcThresholdLR = crate::FieldReader;
#[doc = "Field `DPC_THRESHOLD_L` writer - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DpcThresholdLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_THRESHOLD_H` reader - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DpcThresholdHR = crate::FieldReader;
#[doc = "Field `DPC_THRESHOLD_H` writer - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
pub type DpcThresholdHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPC_FACTOR_DARK` reader - this field configures the dynamic correction method 1 dark factor"]
pub type DpcFactorDarkR = crate::FieldReader;
#[doc = "Field `DPC_FACTOR_DARK` writer - this field configures the dynamic correction method 1 dark factor"]
pub type DpcFactorDarkW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DPC_FACTOR_BRIG` reader - this field configures the dynamic correction method 1 bright factor"]
pub type DpcFactorBrigR = crate::FieldReader;
#[doc = "Field `DPC_FACTOR_BRIG` writer - this field configures the dynamic correction method 1 bright factor"]
pub type DpcFactorBrigW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_l(&self) -> DpcThresholdLR {
        DpcThresholdLR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_h(&self) -> DpcThresholdHR {
        DpcThresholdHR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - this field configures the dynamic correction method 1 dark factor"]
    #[inline(always)]
    pub fn dpc_factor_dark(&self) -> DpcFactorDarkR {
        DpcFactorDarkR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - this field configures the dynamic correction method 1 bright factor"]
    #[inline(always)]
    pub fn dpc_factor_brig(&self) -> DpcFactorBrigR {
        DpcFactorBrigR::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this bit configures the threshold to detect black img in check mode, or the low threshold(use 8 bit 0~255) in dyn method 0, or the low threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_l(&mut self) -> DpcThresholdLW<'_, DpcConfSpec> {
        DpcThresholdLW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this bit configures the threshold to detect white img in check mode, or the high threshold(use 8 bit 0~255) in dyn method 0, or the high threshold factor (use 5 bit 10000-> 16/16, 00001->1/16, 0/16~16/16) in dyn method 1"]
    #[inline(always)]
    pub fn dpc_threshold_h(&mut self) -> DpcThresholdHW<'_, DpcConfSpec> {
        DpcThresholdHW::new(self, 8)
    }
    #[doc = "Bits 16:21 - this field configures the dynamic correction method 1 dark factor"]
    #[inline(always)]
    pub fn dpc_factor_dark(&mut self) -> DpcFactorDarkW<'_, DpcConfSpec> {
        DpcFactorDarkW::new(self, 16)
    }
    #[doc = "Bits 22:27 - this field configures the dynamic correction method 1 bright factor"]
    #[inline(always)]
    pub fn dpc_factor_brig(&mut self) -> DpcFactorBrigW<'_, DpcConfSpec> {
        DpcFactorBrigW::new(self, 22)
    }
}
#[doc = "DPC parameter config register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpcConfSpec;
impl crate::RegisterSpec for DpcConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_conf::R`](R) reader structure"]
impl crate::Readable for DpcConfSpec {}
#[doc = "`write(|w| ..)` method takes [`dpc_conf::W`](W) writer structure"]
impl crate::Writable for DpcConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPC_CONF to value 0x0410_3030"]
impl crate::Resettable for DpcConfSpec {
    const RESET_VALUE: u32 = 0x0410_3030;
}
