CREATE TABLE "jobs" (
  "job_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "completion_record_id" uuid,
  "created_by_id" uuid,
  "status" varchar(20) NOT NULL,
  "failed_count" int4 NOT NULL DEFAULT 0,
  PRIMARY KEY ("job_id")
);
