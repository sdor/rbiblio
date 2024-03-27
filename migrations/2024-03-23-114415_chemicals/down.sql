-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS public.chemicals_name_of_substance_tsv_gin;
DROP INDEX IF EXISTS public.chemicals_year_idx;
DROP TRIGGER IF EXISTS chemicals_name_of_substance_tsv_trigger ON public.chemicals;
DROP TABLE IF EXISTS chemicals;

