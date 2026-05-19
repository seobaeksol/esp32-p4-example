#[doc = "Register `I2C0_CONF` reader"]
pub type R = crate::R<I2c0ConfSpec>;
#[doc = "Register `I2C0_CONF` writer"]
pub type W = crate::W<I2c0ConfSpec>;
#[doc = "Field `I2C0_CONF` reader - need des"]
pub type I2c0ConfR = crate::FieldReader<u32>;
#[doc = "Field `I2C0_CONF` writer - need des"]
pub type I2c0ConfW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `I2C0_STATUS` reader - need des"]
pub type I2c0StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn i2c0_conf(&self) -> I2c0ConfR {
        I2c0ConfR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn i2c0_status(&self) -> I2c0StatusR {
        I2c0StatusR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn i2c0_conf(&mut self) -> I2c0ConfW<'_, I2c0ConfSpec> {
        I2c0ConfW::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0ConfSpec;
impl crate::RegisterSpec for I2c0ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_conf::R`](R) reader structure"]
impl crate::Readable for I2c0ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_conf::W`](W) writer structure"]
impl crate::Writable for I2c0ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_CONF to value 0"]
impl crate::Resettable for I2c0ConfSpec {}
