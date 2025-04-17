# Component Tree Visualization

This diagram shows the component hierarchy and dependencies in the application.

```mermaid
graph TD
    UserList["UserList"]
    CourseCard["CourseCard"]
    Avatar["Avatar"]
    CourseList["CourseList"]
    Button["Button"]
    Icon["Icon"]
    UserCard["UserCard"]
    App["App"]
    Icon --> Button
    Avatar --> UserCard
    Icon --> UserCard
    UserList --> App
    CourseList --> App
    Button --> App
    Button --> UserList
    UserCard --> UserList
    Button --> CourseList
    CourseCard --> CourseList
    Icon --> CourseCard
```

## Component Details

### Icon

**Description**: Component description

**File**: `src/components/Icon.js`

### App

**Description**: Component description

**File**: `src/App.js`

**Dependencies**:
- UserList
- CourseList
- Button

### UserCard

**Description**: Component description

**File**: `src/components/UserCard.js`

**Dependencies**:
- Avatar
- Icon

### UserList

**Description**: Component description

**File**: `src/components/UserList.js`

**Dependencies**:
- UserCard
- Button

### Button

**Description**: Component description

**File**: `src/components/Button.js`

**Dependencies**:
- Icon

### Avatar

**Description**: Component description

**File**: `src/components/Avatar.js`

### CourseList

**Description**: Component description

**File**: `src/components/CourseList.js`

**Dependencies**:
- CourseCard
- Button

### CourseCard

**Description**: Component description

**File**: `src/components/CourseCard.js`

**Dependencies**:
- Icon

