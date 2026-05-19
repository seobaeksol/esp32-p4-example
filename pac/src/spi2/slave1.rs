#[doc = "Register `SLAVE1` reader"]
pub type R = crate::R<Slave1Spec>;
#[doc = "Register `SLAVE1` writer"]
pub type W = crate::W<Slave1Spec>;
#[doc = "Field `SLV_DATA_BITLEN` reader - The transferred data bit length in SPI slave FD and HD mode."]
pub type SlvDataBitlenR = crate::FieldReader<u32>;
#[doc = "Field `SLV_DATA_BITLEN` writer - The transferred data bit length in SPI slave FD and HD mode."]
pub type SlvDataBitlenW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub type SlvLastCommandR = crate::FieldReader;
#[doc = "Field `SLV_LAST_COMMAND` writer - In the slave mode it is the value of command."]
pub type SlvLastCommandW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SLV_LAST_ADDR` reader - In the slave mode it is the value of address."]
pub type SlvLastAddrR = crate::FieldReader;
#[doc = "Field `SLV_LAST_ADDR` writer - In the slave mode it is the value of address."]
pub type SlvLastAddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn slv_data_bitlen(&self) -> SlvDataBitlenR {
        SlvDataBitlenR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SlvLastCommandR {
        SlvLastCommandR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SlvLastAddrR {
        SlvLastAddrR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - The transferred data bit length in SPI slave FD and HD mode."]
    #[inline(always)]
    pub fn slv_data_bitlen(&mut self) -> SlvDataBitlenW<'_, Slave1Spec> {
        SlvDataBitlenW::new(self, 0)
    }
    #[doc = "Bits 18:25 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&mut self) -> SlvLastCommandW<'_, Slave1Spec> {
        SlvLastCommandW::new(self, 18)
    }
    #[doc = "Bits 26:31 - In the slave mode it is the value of address."]
    #[inline(always)]
    pub fn slv_last_addr(&mut self) -> SlvLastAddrW<'_, Slave1Spec> {
        SlvLastAddrW::new(self, 26)
    }
}
#[doc = "SPI slave control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slave1Spec;
impl crate::RegisterSpec for Slave1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave1::R`](R) reader structure"]
impl crate::Readable for Slave1Spec {}
#[doc = "`write(|w| ..)` method takes [`slave1::W`](W) writer structure"]
impl crate::Writable for Slave1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for Slave1Spec {}
