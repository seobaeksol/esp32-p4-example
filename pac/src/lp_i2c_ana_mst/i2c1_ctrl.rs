#[doc = "Register `I2C1_CTRL` reader"]
pub type R = crate::R<I2c1CtrlSpec>;
#[doc = "Register `I2C1_CTRL` writer"]
pub type W = crate::W<I2c1CtrlSpec>;
#[doc = "Field `I2C1_CTRL` reader - need des"]
pub type I2c1CtrlR = crate::FieldReader<u32>;
#[doc = "Field `I2C1_CTRL` writer - need des"]
pub type I2c1CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `I2C1_BUSY` reader - need des"]
pub type I2c1BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:24 - need des"]
    #[inline(always)]
    pub fn i2c1_ctrl(&self) -> I2c1CtrlR {
        I2c1CtrlR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bit 25 - need des"]
    #[inline(always)]
    pub fn i2c1_busy(&self) -> I2c1BusyR {
        I2c1BusyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:24 - need des"]
    #[inline(always)]
    pub fn i2c1_ctrl(&mut self) -> I2c1CtrlW<'_, I2c1CtrlSpec> {
        I2c1CtrlW::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1CtrlSpec;
impl crate::RegisterSpec for I2c1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_ctrl::R`](R) reader structure"]
impl crate::Readable for I2c1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_ctrl::W`](W) writer structure"]
impl crate::Writable for I2c1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1_CTRL to value 0"]
impl crate::Resettable for I2c1CtrlSpec {}
