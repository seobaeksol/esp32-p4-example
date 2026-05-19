#[doc = "Register `WR_MESSAGE_MEM[%s]` reader"]
pub type R = crate::R<WrMessageMemSpec>;
#[doc = "Register `WR_MESSAGE_MEM[%s]` writer"]
pub type W = crate::W<WrMessageMemSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Message block memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_message_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_message_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrMessageMemSpec;
impl crate::RegisterSpec for WrMessageMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_message_mem::R`](R) reader structure"]
impl crate::Readable for WrMessageMemSpec {}
#[doc = "`write(|w| ..)` method takes [`wr_message_mem::W`](W) writer structure"]
impl crate::Writable for WrMessageMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WR_MESSAGE_MEM[%s] to value 0"]
impl crate::Resettable for WrMessageMemSpec {}
