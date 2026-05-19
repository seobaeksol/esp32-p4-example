#[doc = "Register `AT_CMD_PRECNT_SYNC` reader"]
pub type R = crate::R<AtCmdPrecntSyncSpec>;
#[doc = "Register `AT_CMD_PRECNT_SYNC` writer"]
pub type W = crate::W<AtCmdPrecntSyncSpec>;
#[doc = "Field `PRE_IDLE_NUM` reader - This register is used to configure the idle duration time before the first at_cmd is received by receiver."]
pub type PreIdleNumR = crate::FieldReader<u16>;
#[doc = "Field `PRE_IDLE_NUM` writer - This register is used to configure the idle duration time before the first at_cmd is received by receiver."]
pub type PreIdleNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the idle duration time before the first at_cmd is received by receiver."]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PreIdleNumR {
        PreIdleNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the idle duration time before the first at_cmd is received by receiver."]
    #[inline(always)]
    pub fn pre_idle_num(&mut self) -> PreIdleNumW<'_, AtCmdPrecntSyncSpec> {
        PreIdleNumW::new(self, 0)
    }
}
#[doc = "Pre-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_precnt_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_precnt_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtCmdPrecntSyncSpec;
impl crate::RegisterSpec for AtCmdPrecntSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_precnt_sync::R`](R) reader structure"]
impl crate::Readable for AtCmdPrecntSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_precnt_sync::W`](W) writer structure"]
impl crate::Writable for AtCmdPrecntSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT_CMD_PRECNT_SYNC to value 0x0901"]
impl crate::Resettable for AtCmdPrecntSyncSpec {
    const RESET_VALUE: u32 = 0x0901;
}
