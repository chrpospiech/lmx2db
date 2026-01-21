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

-- Minimal SQL data for testing purposes

-- Data for table: people
INSERT INTO `people` (
    `title`, `first_name`, `middle`, `surname`, `affiliation`, `email`, `phone`, `mobile`
) VALUES (
    'Dr.', 'Christoph', '', 'Pospiech', 'retiree', 'pospiech-HD@t-online.de',
    '+49-351-86269826', '+49-1511-910-4597'
);
SET @person_id = LAST_INSERT_ID();

-- Data for table: clusters
INSERT INTO `clusters` (
    `name`, `owner`, `accessinfo`
) VALUES
('Lenox', 'Lenovo', 'ssh lenox'),
('thinkpad', 'Christoph Pospiech', 'no public access');
SET @cluster_id_lenox = (SELECT `id` FROM `clusters` WHERE `name`='Lenox');
SET @cluster_id_thinkpad = (SELECT `id` FROM `clusters` WHERE `name`='thinkpad');


-- Data for table: userids
INSERT INTO `userids` (
    `name`, `clid`, `pid`
) VALUES
('cp', @cluster_id_thinkpad, @person_id),
('xcpospiech', @cluster_id_lenox, @person_id);
