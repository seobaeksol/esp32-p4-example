#[doc = "Register `DBADDR` reader"]
pub type R = crate::R<DbaddrSpec>;
#[doc = "Register `DBADDR` writer"]
pub type W = crate::W<DbaddrSpec>;
#[doc = "Field `DBADDR` reader - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
pub type DbaddrR = crate::FieldReader<u32>;
#[doc = "Field `DBADDR` writer - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
pub type DbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
    #[inline(always)]
    pub fn dbaddr(&self) -> DbaddrR {
        DbaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Descriptor List. Contains the base address of the First Descriptor. The LSB bits \\[1:0\\] are ignored and taken as all-zero by the IDMAC internally. Hence these LSB bits may be treated as read-only."]
    #[inline(always)]
    pub fn dbaddr(&mut self) -> DbaddrW<'_, DbaddrSpec> {
        DbaddrW::new(self, 0)
    }
}
#[doc = "Descriptor base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbaddrSpec;
impl crate::RegisterSpec for DbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbaddr::R`](R) reader structure"]
impl crate::Readable for DbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbaddr::W`](W) writer structure"]
impl crate::Writable for DbaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBADDR to value 0"]
impl crate::Resettable for DbaddrSpec {}
