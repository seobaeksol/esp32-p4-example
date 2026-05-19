#[doc = "Register `AXI_ID0` reader"]
pub type R = crate::R<AxiId0Spec>;
#[doc = "Register `AXI_ID0` writer"]
pub type W = crate::W<AxiId0Spec>;
#[doc = "Field `CH1_AXI_READ_ID_SUFFIX` reader - NA"]
pub type Ch1AxiReadIdSuffixR = crate::BitReader;
#[doc = "Field `CH1_AXI_READ_ID_SUFFIX` writer - NA"]
pub type Ch1AxiReadIdSuffixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_AXI_WRITE_ID_SUFFIX` reader - NA"]
pub type Ch1AxiWriteIdSuffixR = crate::BitReader;
#[doc = "Field `CH1_AXI_WRITE_ID_SUFFIX` writer - NA"]
pub type Ch1AxiWriteIdSuffixW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_axi_read_id_suffix(&self) -> Ch1AxiReadIdSuffixR {
        Ch1AxiReadIdSuffixR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_axi_write_id_suffix(&self) -> Ch1AxiWriteIdSuffixR {
        Ch1AxiWriteIdSuffixR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_axi_read_id_suffix(&mut self) -> Ch1AxiReadIdSuffixW<'_, AxiId0Spec> {
        Ch1AxiReadIdSuffixW::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_axi_write_id_suffix(&mut self) -> Ch1AxiWriteIdSuffixW<'_, AxiId0Spec> {
        Ch1AxiWriteIdSuffixW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_id0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_id0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiId0Spec;
impl crate::RegisterSpec for AxiId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_id0::R`](R) reader structure"]
impl crate::Readable for AxiId0Spec {}
#[doc = "`write(|w| ..)` method takes [`axi_id0::W`](W) writer structure"]
impl crate::Writable for AxiId0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_ID0 to value 0"]
impl crate::Resettable for AxiId0Spec {}
