#[doc = "Register `BLC_VALUE` reader"]
pub type R = crate::R<BlcValueSpec>;
#[doc = "Register `BLC_VALUE` writer"]
pub type W = crate::W<BlcValueSpec>;
#[doc = "Field `BLC_R3_VALUE` reader - this field configures the black level of bottom right channel of bayer img"]
pub type BlcR3ValueR = crate::FieldReader;
#[doc = "Field `BLC_R3_VALUE` writer - this field configures the black level of bottom right channel of bayer img"]
pub type BlcR3ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R2_VALUE` reader - this field configures the black level of bottom left channel of bayer img"]
pub type BlcR2ValueR = crate::FieldReader;
#[doc = "Field `BLC_R2_VALUE` writer - this field configures the black level of bottom left channel of bayer img"]
pub type BlcR2ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R1_VALUE` reader - this field configures the black level of top right channel of bayer img"]
pub type BlcR1ValueR = crate::FieldReader;
#[doc = "Field `BLC_R1_VALUE` writer - this field configures the black level of top right channel of bayer img"]
pub type BlcR1ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R0_VALUE` reader - this field configures the black level of top left channel of bayer img"]
pub type BlcR0ValueR = crate::FieldReader;
#[doc = "Field `BLC_R0_VALUE` writer - this field configures the black level of top left channel of bayer img"]
pub type BlcR0ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the black level of bottom right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r3_value(&self) -> BlcR3ValueR {
        BlcR3ValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the black level of bottom left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r2_value(&self) -> BlcR2ValueR {
        BlcR2ValueR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the black level of top right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r1_value(&self) -> BlcR1ValueR {
        BlcR1ValueR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the black level of top left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r0_value(&self) -> BlcR0ValueR {
        BlcR0ValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the black level of bottom right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r3_value(&mut self) -> BlcR3ValueW<'_, BlcValueSpec> {
        BlcR3ValueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the black level of bottom left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r2_value(&mut self) -> BlcR2ValueW<'_, BlcValueSpec> {
        BlcR2ValueW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the black level of top right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r1_value(&mut self) -> BlcR1ValueW<'_, BlcValueSpec> {
        BlcR1ValueW::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the black level of top left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r0_value(&mut self) -> BlcR0ValueW<'_, BlcValueSpec> {
        BlcR0ValueW::new(self, 24)
    }
}
#[doc = "blc black level register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlcValueSpec;
impl crate::RegisterSpec for BlcValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blc_value::R`](R) reader structure"]
impl crate::Readable for BlcValueSpec {}
#[doc = "`write(|w| ..)` method takes [`blc_value::W`](W) writer structure"]
impl crate::Writable for BlcValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLC_VALUE to value 0"]
impl crate::Resettable for BlcValueSpec {}
