#[doc = "Register `LINK_CONF` reader"]
pub type R = crate::R<LinkConfSpec>;
#[doc = "Register `LINK_CONF` writer"]
pub type W = crate::W<LinkConfSpec>;
#[doc = "Field `INLINK_AUTO_RET` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type InlinkAutoRetR = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type InlinkAutoRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP` reader - Set this bit to stop dealing with the inlink descriptors."]
pub type InlinkStopR = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type InlinkStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START` reader - Set this bit to start dealing with the inlink descriptors."]
pub type InlinkStartR = crate::BitReader;
#[doc = "Field `INLINK_START` writer - Set this bit to start dealing with the inlink descriptors."]
pub type InlinkStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART` reader - Set this bit to mount a new inlink descriptor."]
pub type InlinkRestartR = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - Set this bit to mount a new inlink descriptor."]
pub type InlinkRestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type InlinkParkR = crate::BitReader;
impl R {
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> InlinkAutoRetR {
        InlinkAutoRetR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> InlinkStopR {
        InlinkStopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start(&self) -> InlinkStartR {
        InlinkStartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&self) -> InlinkRestartR {
        InlinkRestartR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park(&self) -> InlinkParkR {
        InlinkParkR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&mut self) -> InlinkAutoRetW<'_, LinkConfSpec> {
        InlinkAutoRetW::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&mut self) -> InlinkStopW<'_, LinkConfSpec> {
        InlinkStopW::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start(&mut self) -> InlinkStartW<'_, LinkConfSpec> {
        InlinkStartW::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&mut self) -> InlinkRestartW<'_, LinkConfSpec> {
        InlinkRestartW::new(self, 23)
    }
}
#[doc = "RX CHx in_link dscr ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkConfSpec;
impl crate::RegisterSpec for LinkConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_conf::R`](R) reader structure"]
impl crate::Readable for LinkConfSpec {}
#[doc = "`write(|w| ..)` method takes [`link_conf::W`](W) writer structure"]
impl crate::Writable for LinkConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK_CONF to value 0x0110_0000"]
impl crate::Resettable for LinkConfSpec {
    const RESET_VALUE: u32 = 0x0110_0000;
}
