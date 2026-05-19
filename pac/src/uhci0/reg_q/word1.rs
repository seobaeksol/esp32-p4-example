#[doc = "Register `WORD1` reader"]
pub type R = crate::R<Word1Spec>;
#[doc = "Register `WORD1` writer"]
pub type W = crate::W<Word1Spec>;
#[doc = "Field `SEND_WORD` reader - Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SendWordR = crate::FieldReader<u32>;
#[doc = "Field `SEND_WORD` writer - Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SendWordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_word(&self) -> SendWordR {
        SendWordR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Serves as quick sending register in specified mode in UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_word(&mut self) -> SendWordW<'_, Word1Spec> {
        SendWordW::new(self, 0)
    }
}
#[doc = "UHCI Q0_WORD1 Quick Send Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Word1Spec;
impl crate::RegisterSpec for Word1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`word1::R`](R) reader structure"]
impl crate::Readable for Word1Spec {}
#[doc = "`write(|w| ..)` method takes [`word1::W`](W) writer structure"]
impl crate::Writable for Word1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORD1 to value 0"]
impl crate::Resettable for Word1Spec {}
