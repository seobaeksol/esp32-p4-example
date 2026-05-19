#[doc = "Register `AT_CMD_POSTCNT` reader"]
pub type R = crate::R<AtCmdPostcntSpec>;
#[doc = "Register `AT_CMD_POSTCNT` writer"]
pub type W = crate::W<AtCmdPostcntSpec>;
#[doc = "Field `POST_IDLE_NUM` reader - This register is used to configure the duration time between the last at_cmd and the next data."]
pub type PostIdleNumR = crate::FieldReader<u16>;
#[doc = "Field `POST_IDLE_NUM` writer - This register is used to configure the duration time between the last at_cmd and the next data."]
pub type PostIdleNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the last at_cmd and the next data."]
    #[inline(always)]
    pub fn post_idle_num(&self) -> PostIdleNumR {
        PostIdleNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the last at_cmd and the next data."]
    #[inline(always)]
    pub fn post_idle_num(&mut self) -> PostIdleNumW<'_, AtCmdPostcntSpec> {
        PostIdleNumW::new(self, 0)
    }
}
#[doc = "Post-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_postcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_postcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtCmdPostcntSpec;
impl crate::RegisterSpec for AtCmdPostcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_postcnt::R`](R) reader structure"]
impl crate::Readable for AtCmdPostcntSpec {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_postcnt::W`](W) writer structure"]
impl crate::Writable for AtCmdPostcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT_CMD_POSTCNT to value 0x0901"]
impl crate::Resettable for AtCmdPostcntSpec {
    const RESET_VALUE: u32 = 0x0901;
}
