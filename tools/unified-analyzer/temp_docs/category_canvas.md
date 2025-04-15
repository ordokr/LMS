# EportfolioCategory

## Description

frozen_string_literal: true

Copyright (C) 2011 - present Instructure, Inc.

This file is part of Canvas.

Canvas is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, version 3 of the License.

Canvas is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <http://www.gnu.org/licenses/>.


## Relationships

- has_many :eportfolio_entries
- belongs_to :eportfolio

## Methods

### infer_unique_slug

frozen_string_literal: true

#
# Copyright (C) 2011 - present Instructure, Inc.
#
# This file is part of Canvas.
#
# Canvas is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, version 3 of the License.
#
# Canvas is distributed in the hope that it will be useful, but WITHOUT ANY
# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
# A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
# details.
#
# You should have received a copy of the GNU Affero General Public License along
# with this program. If not, see <http://www.gnu.org/licenses/>.
#

class EportfolioCategory < ActiveRecord::Base
  attr_readonly :eportfolio_id

  has_many :eportfolio_entries, -> { ordered }, dependent: :destroy
  belongs_to :eportfolio

  before_save :infer_unique_slug, if: ->(category) { category.slug.blank? || category.will_save_change_to_name? }
  after_save :check_for_spam, if: -> { eportfolio.needs_spam_review? }

  validates :eportfolio_id, presence: true
  validates :name, length: { maximum: maximum_string_length, allow_blank: true }

  acts_as_list scope: :eportfolio

