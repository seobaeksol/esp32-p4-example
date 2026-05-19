#[doc = "Register `DMARINTWDTIMER` reader"]
pub type R = crate::R<DmarintwdtimerSpec>;
#[doc = "Register `DMARINTWDTIMER` writer"]
pub type W = crate::W<DmarintwdtimerSpec>;
#[doc = "Field `RIWTC` reader - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
pub type RiwtcR = crate::FieldReader;
#[doc = "Field `RIWTC` writer - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
pub type RiwtcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
    #[inline(always)]
    pub fn riwtc(&self) -> RiwtcR {
        RiwtcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI(RECV_INT) status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\] of any received frame."]
    #[inline(always)]
    pub fn riwtc(&mut self) -> RiwtcW<'_, DmarintwdtimerSpec> {
        RiwtcW::new(self, 0)
    }
}
#[doc = "Watchdog timer count on receive\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarintwdtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarintwdtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarintwdtimerSpec;
impl crate::RegisterSpec for DmarintwdtimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarintwdtimer::R`](R) reader structure"]
impl crate::Readable for DmarintwdtimerSpec {}
#[doc = "`write(|w| ..)` method takes [`dmarintwdtimer::W`](W) writer structure"]
impl crate::Writable for DmarintwdtimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARINTWDTIMER to value 0"]
impl crate::Resettable for DmarintwdtimerSpec {}
