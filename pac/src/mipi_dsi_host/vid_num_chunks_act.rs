#[doc = "Register `VID_NUM_CHUNKS_ACT` reader"]
pub type R = crate::R<VidNumChunksActSpec>;
#[doc = "Field `VID_NUM_CHUNKS_ACT` reader - NA"]
pub type VidNumChunksActR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NA"]
    #[inline(always)]
    pub fn vid_num_chunks_act(&self) -> VidNumChunksActR {
        VidNumChunksActR::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_num_chunks_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidNumChunksActSpec;
impl crate::RegisterSpec for VidNumChunksActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_num_chunks_act::R`](R) reader structure"]
impl crate::Readable for VidNumChunksActSpec {}
#[doc = "`reset()` method sets VID_NUM_CHUNKS_ACT to value 0"]
impl crate::Resettable for VidNumChunksActSpec {}
