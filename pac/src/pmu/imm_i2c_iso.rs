#[doc = "Register `IMM_I2C_ISO` writer"]
pub type W = crate::W<ImmI2cIsoSpec>;
#[doc = "Field `TIE_HIGH_I2C_ISO_EN` writer - need_des"]
pub type TieHighI2cIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_I2C_ISO_EN` writer - need_des"]
pub type TieLowI2cIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_i2c_iso_en(&mut self) -> TieHighI2cIsoEnW<'_, ImmI2cIsoSpec> {
        TieHighI2cIsoEnW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tie_low_i2c_iso_en(&mut self) -> TieLowI2cIsoEnW<'_, ImmI2cIsoSpec> {
        TieLowI2cIsoEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_i2c_iso::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImmI2cIsoSpec;
impl crate::RegisterSpec for ImmI2cIsoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_i2c_iso::W`](W) writer structure"]
impl crate::Writable for ImmI2cIsoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_I2C_ISO to value 0"]
impl crate::Resettable for ImmI2cIsoSpec {}
