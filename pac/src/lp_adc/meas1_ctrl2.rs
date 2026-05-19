#[doc = "Register `MEAS1_CTRL2` reader"]
pub type R = crate::R<Meas1Ctrl2Spec>;
#[doc = "Register `MEAS1_CTRL2` writer"]
pub type W = crate::W<Meas1Ctrl2Spec>;
#[doc = "Field `MEAS1_DATA_SAR` reader - SAR ADC1 data."]
pub type Meas1DataSarR = crate::FieldReader<u16>;
#[doc = "Field `MEAS1_DONE_SAR` reader - SAR ADC1 conversion done indication."]
pub type Meas1DoneSarR = crate::BitReader;
#[doc = "Field `MEAS1_START_SAR` reader - SAR ADC1 controller (in RTC) starts conversion."]
pub type Meas1StartSarR = crate::BitReader;
#[doc = "Field `MEAS1_START_SAR` writer - SAR ADC1 controller (in RTC) starts conversion."]
pub type Meas1StartSarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEAS1_START_FORCE` reader - 1: SAR ADC1 controller (in RTC) is started by SW."]
pub type Meas1StartForceR = crate::BitReader;
#[doc = "Field `MEAS1_START_FORCE` writer - 1: SAR ADC1 controller (in RTC) is started by SW."]
pub type Meas1StartForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_EN_PAD` reader - SAR ADC1 pad enable bitmap."]
pub type Sar1EnPadR = crate::FieldReader<u16>;
#[doc = "Field `SAR1_EN_PAD` writer - SAR ADC1 pad enable bitmap."]
pub type Sar1EnPadW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR1_EN_PAD_FORCE` reader - 1: SAR ADC1 pad enable bitmap is controlled by SW."]
pub type Sar1EnPadForceR = crate::BitReader;
#[doc = "Field `SAR1_EN_PAD_FORCE` writer - 1: SAR ADC1 pad enable bitmap is controlled by SW."]
pub type Sar1EnPadForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - SAR ADC1 data."]
    #[inline(always)]
    pub fn meas1_data_sar(&self) -> Meas1DataSarR {
        Meas1DataSarR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - SAR ADC1 conversion done indication."]
    #[inline(always)]
    pub fn meas1_done_sar(&self) -> Meas1DoneSarR {
        Meas1DoneSarR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion."]
    #[inline(always)]
    pub fn meas1_start_sar(&self) -> Meas1StartSarR {
        Meas1StartSarR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW."]
    #[inline(always)]
    pub fn meas1_start_force(&self) -> Meas1StartForceR {
        Meas1StartForceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap."]
    #[inline(always)]
    pub fn sar1_en_pad(&self) -> Sar1EnPadR {
        Sar1EnPadR::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW."]
    #[inline(always)]
    pub fn sar1_en_pad_force(&self) -> Sar1EnPadForceR {
        Sar1EnPadForceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion."]
    #[inline(always)]
    pub fn meas1_start_sar(&mut self) -> Meas1StartSarW<'_, Meas1Ctrl2Spec> {
        Meas1StartSarW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW."]
    #[inline(always)]
    pub fn meas1_start_force(&mut self) -> Meas1StartForceW<'_, Meas1Ctrl2Spec> {
        Meas1StartForceW::new(self, 18)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap."]
    #[inline(always)]
    pub fn sar1_en_pad(&mut self) -> Sar1EnPadW<'_, Meas1Ctrl2Spec> {
        Sar1EnPadW::new(self, 19)
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW."]
    #[inline(always)]
    pub fn sar1_en_pad_force(&mut self) -> Sar1EnPadForceW<'_, Meas1Ctrl2Spec> {
        Sar1EnPadForceW::new(self, 31)
    }
}
#[doc = "ADC1 configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas1_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas1_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Meas1Ctrl2Spec;
impl crate::RegisterSpec for Meas1Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meas1_ctrl2::R`](R) reader structure"]
impl crate::Readable for Meas1Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`meas1_ctrl2::W`](W) writer structure"]
impl crate::Writable for Meas1Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEAS1_CTRL2 to value 0"]
impl crate::Resettable for Meas1Ctrl2Spec {}
