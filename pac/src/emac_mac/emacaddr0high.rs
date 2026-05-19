#[doc = "Register `EMACADDR0HIGH` reader"]
pub type R = crate::R<Emacaddr0highSpec>;
#[doc = "Register `EMACADDR0HIGH` writer"]
pub type W = crate::W<Emacaddr0highSpec>;
#[doc = "Field `ADDRESS0_HI` reader - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type Address0HiR = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS0_HI` writer - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type Address0HiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRESS_ENABLE0` reader - This bit is always set to 1."]
pub type AddressEnable0R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE0` writer - This bit is always set to 1."]
pub type AddressEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub fn address0_hi(&self) -> Address0HiR {
        Address0HiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - This bit is always set to 1."]
    #[inline(always)]
    pub fn address_enable0(&self) -> AddressEnable0R {
        AddressEnable0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub fn address0_hi(&mut self) -> Address0HiW<'_, Emacaddr0highSpec> {
        Address0HiW::new(self, 0)
    }
    #[doc = "Bit 31 - This bit is always set to 1."]
    #[inline(always)]
    pub fn address_enable0(&mut self) -> AddressEnable0W<'_, Emacaddr0highSpec> {
        AddressEnable0W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr0high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr0high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emacaddr0highSpec;
impl crate::RegisterSpec for Emacaddr0highSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr0high::R`](R) reader structure"]
impl crate::Readable for Emacaddr0highSpec {}
#[doc = "`write(|w| ..)` method takes [`emacaddr0high::W`](W) writer structure"]
impl crate::Writable for Emacaddr0highSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR0HIGH to value 0"]
impl crate::Resettable for Emacaddr0highSpec {}
