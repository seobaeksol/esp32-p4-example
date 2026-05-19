#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `START` writer - Write 1 to start caculation of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAD_DONE` writer - Write 1 to input load done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type LoadDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_DONE` writer - Write 1 to input get done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type GetDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to start caculation of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, StartSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to input load done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    pub fn load_done(&mut self) -> LoadDoneW<'_, StartSpec> {
        LoadDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to input get done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    pub fn get_done(&mut self) -> GetDoneW<'_, StartSpec> {
        GetDoneW::new(self, 2)
    }
}
#[doc = "ECDSA start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}
