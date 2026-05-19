#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Field `OP_CODE` reader - 0x5A5A: programming operation command 0x5AA5: read operation command."]
pub type OpCodeR = crate::FieldReader<u16>;
#[doc = "Field `OP_CODE` writer - 0x5A5A: programming operation command 0x5AA5: read operation command."]
pub type OpCodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command."]
    #[inline(always)]
    pub fn op_code(&self) -> OpCodeR {
        OpCodeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command."]
    #[inline(always)]
    pub fn op_code(&mut self) -> OpCodeW<'_, ConfSpec> {
        OpCodeW::new(self, 0)
    }
}
#[doc = "eFuse operation mode configuraiton register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {}
