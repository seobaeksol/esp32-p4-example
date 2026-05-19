#[doc = "Register `CH_GAMMA_RANGE%s` reader"]
pub type R = crate::R<ChGammaRangeSpec>;
#[doc = "Register `CH_GAMMA_RANGE%s` writer"]
pub type W = crate::W<ChGammaRangeSpec>;
#[doc = "Field `DUTY_INC` reader - Duty increase or decrease"]
pub type DutyIncR = crate::BitReader;
#[doc = "Field `DUTY_INC` writer - Duty increase or decrease"]
pub type DutyIncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CYCLE` reader - Duty cycle"]
pub type DutyCycleR = crate::FieldReader<u16>;
#[doc = "Field `DUTY_CYCLE` writer - Duty cycle"]
pub type DutyCycleW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SCALE` reader - Gamma scale"]
pub type ScaleR = crate::FieldReader<u16>;
#[doc = "Field `SCALE` writer - Gamma scale"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_NUM` reader - Duty number"]
pub type DutyNumR = crate::FieldReader<u16>;
#[doc = "Field `DUTY_NUM` writer - Duty number"]
pub type DutyNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Duty increase or decrease"]
    #[inline(always)]
    pub fn duty_inc(&self) -> DutyIncR {
        DutyIncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Duty cycle"]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DutyCycleR {
        DutyCycleR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - Gamma scale"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - Duty number"]
    #[inline(always)]
    pub fn duty_num(&self) -> DutyNumR {
        DutyNumR::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Duty increase or decrease"]
    #[inline(always)]
    pub fn duty_inc(&mut self) -> DutyIncW<'_, ChGammaRangeSpec> {
        DutyIncW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Duty cycle"]
    #[inline(always)]
    pub fn duty_cycle(&mut self) -> DutyCycleW<'_, ChGammaRangeSpec> {
        DutyCycleW::new(self, 1)
    }
    #[doc = "Bits 11:20 - Gamma scale"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<'_, ChGammaRangeSpec> {
        ScaleW::new(self, 11)
    }
    #[doc = "Bits 21:30 - Duty number"]
    #[inline(always)]
    pub fn duty_num(&mut self) -> DutyNumW<'_, ChGammaRangeSpec> {
        DutyNumW::new(self, 21)
    }
}
#[doc = "Gamma range word %s (channel = %s / 16, range = %s mod 16)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_range::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_range::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChGammaRangeSpec;
impl crate::RegisterSpec for ChGammaRangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_gamma_range::R`](R) reader structure"]
impl crate::Readable for ChGammaRangeSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_gamma_range::W`](W) writer structure"]
impl crate::Writable for ChGammaRangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_GAMMA_RANGE%s to value 0"]
impl crate::Resettable for ChGammaRangeSpec {}
