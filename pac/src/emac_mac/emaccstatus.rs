#[doc = "Register `EMACCSTATUS` reader"]
pub type R = crate::R<EmaccstatusSpec>;
#[doc = "Field `LINK_MODE` reader - This bit indicates the current mode of operation of the link: 1'b0: Half-duplex mode. 1'b1: Full-duplex mode."]
pub type LinkModeR = crate::BitReader;
#[doc = "Field `LINK_SPEED` reader - This bit indicates the current speed of the link: 2'b00: 2.5 MHz. 2'b01: 25 MHz. 2'b10: 125 MHz."]
pub type LinkSpeedR = crate::FieldReader;
#[doc = "Field `JABBER_TIMEOUT` reader - This bit indicates whether there is jabber timeout error (1'b1) in the received Frame."]
pub type JabberTimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates the current mode of operation of the link: 1'b0: Half-duplex mode. 1'b1: Full-duplex mode."]
    #[inline(always)]
    pub fn link_mode(&self) -> LinkModeR {
        LinkModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This bit indicates the current speed of the link: 2'b00: 2.5 MHz. 2'b01: 25 MHz. 2'b10: 125 MHz."]
    #[inline(always)]
    pub fn link_speed(&self) -> LinkSpeedR {
        LinkSpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - This bit indicates whether there is jabber timeout error (1'b1) in the received Frame."]
    #[inline(always)]
    pub fn jabber_timeout(&self) -> JabberTimeoutR {
        JabberTimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Link communication status\n\nYou can [`read`](crate::Reg::read) this register and get [`emaccstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmaccstatusSpec;
impl crate::RegisterSpec for EmaccstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emaccstatus::R`](R) reader structure"]
impl crate::Readable for EmaccstatusSpec {}
#[doc = "`reset()` method sets EMACCSTATUS to value 0"]
impl crate::Resettable for EmaccstatusSpec {}
