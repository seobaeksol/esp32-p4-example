#[doc = "Register `SLAVE_ADDR` reader"]
pub type R = crate::R<SlaveAddrSpec>;
#[doc = "Register `SLAVE_ADDR` writer"]
pub type W = crate::W<SlaveAddrSpec>;
#[doc = "Field `SLAVE_ADDR` reader - Configure the slave address of I2C Slave.\\\\"]
pub type SlaveAddrR = crate::FieldReader<u16>;
#[doc = "Field `SLAVE_ADDR` writer - Configure the slave address of I2C Slave.\\\\"]
pub type SlaveAddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADDR_10BIT_EN` reader - Configures to enable the slave 10-bit addressing mode in master mode.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type Addr10bitEnR = crate::BitReader;
#[doc = "Field `ADDR_10BIT_EN` writer - Configures to enable the slave 10-bit addressing mode in master mode.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type Addr10bitEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Configure the slave address of I2C Slave.\\\\"]
    #[inline(always)]
    pub fn slave_addr(&self) -> SlaveAddrR {
        SlaveAddrR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Configures to enable the slave 10-bit addressing mode in master mode.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn addr_10bit_en(&self) -> Addr10bitEnR {
        Addr10bitEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Configure the slave address of I2C Slave.\\\\"]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SlaveAddrW<'_, SlaveAddrSpec> {
        SlaveAddrW::new(self, 0)
    }
    #[doc = "Bit 31 - Configures to enable the slave 10-bit addressing mode in master mode.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn addr_10bit_en(&mut self) -> Addr10bitEnW<'_, SlaveAddrSpec> {
        Addr10bitEnW::new(self, 31)
    }
}
#[doc = "Local slave address setting\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlaveAddrSpec;
impl crate::RegisterSpec for SlaveAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_addr::R`](R) reader structure"]
impl crate::Readable for SlaveAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`slave_addr::W`](W) writer structure"]
impl crate::Writable for SlaveAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLAVE_ADDR to value 0"]
impl crate::Resettable for SlaveAddrSpec {}
