CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- ----------------------------
-- Function structure for set_current_timestamp_updated_at
-- ----------------------------
-- DROP FUNCTION IF EXISTS "public"."set_current_timestamp_updated_at"();
CREATE OR REPLACE FUNCTION "public"."set_current_timestamp_updated_at"()
  RETURNS "pg_catalog"."trigger" AS $BODY$
DECLARE
  _new record;
BEGIN
  _new := NEW;
  _new."updated_at" = NOW();
  RETURN _new;
END;
$BODY$
  LANGUAGE plpgsql VOLATILE
  COST 100;

-- ----------------------------
-- Function structure for update_modified_time
-- ----------------------------
-- DROP FUNCTION IF EXISTS "public"."update_modified_time"();
CREATE OR REPLACE FUNCTION "public"."update_modified_time"()
  RETURNS "pg_catalog"."trigger" AS $BODY$
BEGIN
    NEW.modified_time = now();
    RETURN NEW;
END;
$BODY$
  LANGUAGE plpgsql VOLATILE
  COST 100;

-- ----------------------------
-- Function structure for update_updated_on_user_task
-- ----------------------------
-- DROP FUNCTION IF EXISTS "public"."update_updated_on_user_task"();
CREATE OR REPLACE FUNCTION "public"."update_updated_on_user_task"()
  RETURNS "pg_catalog"."trigger" AS $BODY$
BEGIN
    NEW.last_modified_time = now();
    RETURN NEW;
END;
$BODY$
  LANGUAGE plpgsql VOLATILE
  COST 100;

-- ----------------------------
-- Triggers structure for table flow_draft
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."flow_draft"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_updated_on_user_task"();


-- ----------------------------
-- Triggers structure for table flow_instance
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."flow_instance"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_updated_on_user_task"();


-- ----------------------------
-- Triggers structure for table flow_instance_billing
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."flow_instance_billing"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_modified_time"();

-- ----------------------------
-- Triggers structure for table node_instance
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."node_instance"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_updated_on_user_task"();

-- ----------------------------
-- Triggers structure for table node_instance_billing
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."node_instance_billing"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_modified_time"();

-- ----------------------------
-- Triggers structure for table project
-- ----------------------------
CREATE OR REPLACE TRIGGER "set_public_project_updated_at" BEFORE UPDATE ON "public"."project"
FOR EACH ROW
EXECUTE PROCEDURE "public"."set_current_timestamp_updated_at"();
COMMENT ON TRIGGER "set_public_project_updated_at" ON "public"."project" IS 'trigger to set value of column "updated_at" to current timestamp on row update';

-- ----------------------------
-- Triggers structure for table project_user
-- ----------------------------
CREATE OR REPLACE TRIGGER "set_public_project_user_updated_at" BEFORE UPDATE ON "public"."project_user"
FOR EACH ROW
EXECUTE PROCEDURE "public"."set_current_timestamp_updated_at"();
COMMENT ON TRIGGER "set_public_project_user_updated_at" ON "public"."project_user" IS 'trigger to set value of column "updated_at" to current timestamp on row update';

-- ----------------------------
-- Triggers structure for table software_block_list
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."software_block_list"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_updated_on_user_task"();

-- ----------------------------
-- Triggers structure for table task
-- ----------------------------
CREATE OR REPLACE TRIGGER "update_time" BEFORE UPDATE ON "public"."task"
FOR EACH ROW
EXECUTE PROCEDURE "public"."update_updated_on_user_task"();
