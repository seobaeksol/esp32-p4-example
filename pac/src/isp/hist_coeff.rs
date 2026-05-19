#[doc = "Register `HIST_COEFF` reader"]
pub type R = crate::R<HistCoeffSpec>;
#[doc = "Register `HIST_COEFF` writer"]
pub type W = crate::W<HistCoeffSpec>;
#[doc = "Field `B` reader - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type BR = crate::FieldReader;
#[doc = "Field `B` writer - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type BW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `G` reader - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type GR = crate::FieldReader;
#[doc = "Field `G` writer - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type GW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R` reader - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type RR = crate::FieldReader;
#[doc = "Field `R` writer - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
pub type RW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn g(&self) -> GR {
        GR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn b(&mut self) -> BW<'_, HistCoeffSpec> {
        BW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn g(&mut self) -> GW<'_, HistCoeffSpec> {
        GW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256"]
    #[inline(always)]
    pub fn r(&mut self) -> RW<'_, HistCoeffSpec> {
        RW::new(self, 16)
    }
}
#[doc = "histogram rgb to gray coefficients register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_coeff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_coeff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistCoeffSpec;
impl crate::RegisterSpec for HistCoeffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_coeff::R`](R) reader structure"]
impl crate::Readable for HistCoeffSpec {}
#[doc = "`write(|w| ..)` method takes [`hist_coeff::W`](W) writer structure"]
impl crate::Writable for HistCoeffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_COEFF to value 0x0055_5555"]
impl crate::Resettable for HistCoeffSpec {
    const RESET_VALUE: u32 = 0x0055_5555;
}
