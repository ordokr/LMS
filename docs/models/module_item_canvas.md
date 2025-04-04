# that

## Description

frozen_string_literal: true

Copyright (C) 2012 - present Instructure, Inc.

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

This isn't a record on its own, but a module included in other records such
as Attachment and Assignment.

ContextModules contain items indirectly, through ContentTags that contain the
information on position in the module, progression requirements, etc.
module ContextModuleItem
set up the association for the AR

## Relationships

- has_many :context_module_tags
- has_many :context_modules

## Methods

### self

frozen_string_literal: true

#
# Copyright (C) 2012 - present Instructure, Inc.
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

# This isn't a record on its own, but a module included in other records such
# as Attachment and Assignment.
#
# ContextModules contain items indirectly, through ContentTags that contain the
# information on position in the module, progression requirements, etc.
module ContextModuleItem
  # set up the association for the AR class that included this module

### locked_by_module_item

Check if this item is locked for the given user.
  # If we are locked, this will return the module item (ContentTag) that is
  # locking the item for the given user

### self

searches the ContextModuleItems in objs_to_search, in order, for the first
  # context module tag -- returns the tag with id preferred_id if given and it
  # exists
  #
  # If no preferred is found, but more than one tag exists for the same obj, we
  # return nothing, since we can't know which tag is appropriate to return.

