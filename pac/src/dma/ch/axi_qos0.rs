#[doc = "Register `AXI_QOS0` reader"]
pub type R = crate::R<AxiQos0Spec>;
#[doc = "Register `AXI_QOS0` writer"]
pub type W = crate::W<AxiQos0Spec>;
#[doc = "Field `CH1_AXI_AWQOS` reader - NA"]
pub type Ch1AxiAwqosR = crate::FieldReader;
#[doc = "Field `CH1_AXI_AWQOS` writer - NA"]
pub type Ch1AxiAwqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_AXI_ARQOS` reader - NA"]
pub type Ch1AxiArqosR = crate::FieldReader;
#[doc = "Field `CH1_AXI_ARQOS` writer - NA"]
pub type Ch1AxiArqosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn ch1_axi_awqos(&self) -> Ch1AxiAwqosR {
        Ch1AxiAwqosR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    pub fn ch1_axi_arqos(&self) -> Ch1AxiArqosR {
        Ch1AxiArqosR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn ch1_axi_awqos(&mut self) -> Ch1AxiAwqosW<'_, AxiQos0Spec> {
        Ch1AxiAwqosW::new(self, 0)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    pub fn ch1_axi_arqos(&mut self) -> Ch1AxiArqosW<'_, AxiQos0Spec> {
        Ch1AxiArqosW::new(self, 4)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_qos0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_qos0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiQos0Spec;
impl crate::RegisterSpec for AxiQos0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_qos0::R`](R) reader structure"]
impl crate::Readable for AxiQos0Spec {}
#[doc = "`write(|w| ..)` method takes [`axi_qos0::W`](W) writer structure"]
impl crate::Writable for AxiQos0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_QOS0 to value 0"]
impl crate::Resettable for AxiQos0Spec {}
