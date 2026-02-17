-- Copyright (c) 2026 C. Pospiech
--
-- This file is part of lmx2db.
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.

CREATE ALGORITHM=UNDEFINED DEFINER=`cpospiech`@`%` SQL SECURITY INVOKER VIEW `Alphabetic_list_of_customer_cases`  AS SELECT `customer_cases`.`id` AS `ccid`, `project_name`(`customer_cases`.`id`) AS `project`, `code_name`(`customer_cases`.`id`) AS `code`, `code_version`(`customer_cases`.`id`) AS `version`, `testcase_name`(`customer_cases`.`id`) AS `test case` FROM `customer_cases` ORDER BY `project_name`(`customer_cases`.`id`) ASC, `code_name`(`customer_cases`.`id`) ASC, `code_version`(`customer_cases`.`id`) ASC, `testcase_name`(`customer_cases`.`id`) ASC ;
COMMIT;
