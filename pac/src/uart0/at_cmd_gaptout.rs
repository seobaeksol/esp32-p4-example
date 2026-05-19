#[doc = "Register `AT_CMD_GAPTOUT` reader"]
pub type R = crate::R<AtCmdGaptoutSpec>;
#[doc = "Register `AT_CMD_GAPTOUT` writer"]
pub type W = crate::W<AtCmdGaptoutSpec>;
#[doc = "Field `RX_GAP_TOUT` reader - This register is used to configure the duration time between the at_cmd chars."]
pub type RxGapToutR = crate::FieldReader<u16>;
#[doc = "Field `RX_GAP_TOUT` writer - This register is used to configure the duration time between the at_cmd chars."]
pub type RxGapToutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the at_cmd chars."]
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> RxGapToutR {
        RxGapToutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the at_cmd chars."]
    #[inline(always)]
    pub fn rx_gap_tout(&mut self) -> RxGapToutW<'_, AtCmdGaptoutSpec> {
        RxGapToutW::new(self, 0)
    }
}
#[doc = "Timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_gaptout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_gaptout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtCmdGaptoutSpec;
impl crate::RegisterSpec for AtCmdGaptoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_gaptout::R`](R) reader structure"]
impl crate::Readable for AtCmdGaptoutSpec {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_gaptout::W`](W) writer structure"]
impl crate::Writable for AtCmdGaptoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT_CMD_GAPTOUT to value 0x0b"]
impl crate::Resettable for AtCmdGaptoutSpec {
    const RESET_VALUE: u32 = 0x0b;
}
