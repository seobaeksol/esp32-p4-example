#[doc = "Register `HWFC_CONF` reader"]
pub type R = crate::R<HwfcConfSpec>;
#[doc = "Register `HWFC_CONF` writer"]
pub type W = crate::W<HwfcConfSpec>;
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RxFlowThrhdR = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RxFlowThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for UART receiver."]
pub type RxFlowEnR = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for UART receiver."]
pub type RxFlowEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RxFlowThrhdR {
        RxFlowThrhdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RxFlowEnR {
        RxFlowEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RxFlowThrhdW<'_, HwfcConfSpec> {
        RxFlowThrhdW::new(self, 0)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RxFlowEnW<'_, HwfcConfSpec> {
        RxFlowEnW::new(self, 8)
    }
}
#[doc = "Hardware flow-control configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hwfc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwfc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwfcConfSpec;
impl crate::RegisterSpec for HwfcConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwfc_conf::R`](R) reader structure"]
impl crate::Readable for HwfcConfSpec {}
#[doc = "`write(|w| ..)` method takes [`hwfc_conf::W`](W) writer structure"]
impl crate::Writable for HwfcConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HWFC_CONF to value 0"]
impl crate::Resettable for HwfcConfSpec {}
