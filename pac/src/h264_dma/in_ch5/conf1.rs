#[doc = "Register `CONF1` reader"]
pub type R = crate::R<Conf1Spec>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<Conf1Spec>;
#[doc = "Field `BLOCK_START_ADDR` reader - RX Channel 5 destination start address"]
pub type BlockStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `BLOCK_START_ADDR` writer - RX Channel 5 destination start address"]
pub type BlockStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    pub fn block_start_addr(&self) -> BlockStartAddrR {
        BlockStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Channel 5 destination start address"]
    #[inline(always)]
    pub fn block_start_addr(&mut self) -> BlockStartAddrW<'_, Conf1Spec> {
        BlockStartAddrW::new(self, 0)
    }
}
#[doc = "RX CH5 config1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf1Spec;
impl crate::RegisterSpec for Conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for Conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for Conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for Conf1Spec {}
