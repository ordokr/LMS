const CourseCategoryMapper = require('../../../../services/integration/mapping/course-category-mapper');
const db = require('../../../../services/database');

jest.mock('../../../../services/database', () => ({
  query: jest.fn(),
}));

const mockCanvasClient = {
  getCourse: jest.fn(),
};

const mockDiscourseClient = {
  getCategory: jest.fn(),
  createCategory: jest.fn(),
  updateCategory: jest.fn(),
  deleteCategory: jest.fn(),
};

describe('CourseCategoryMapper', () => {
  let mapper;

  beforeEach(() => {
    mapper = new CourseCategoryMapper({
      canvasClient: mockCanvasClient,
      discourseClient: mockDiscourseClient,
    });
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  test('should throw an error if canvasClient is not provided', () => {
    expect(() => new CourseCategoryMapper({ discourseClient: mockDiscourseClient })).toThrow('Canvas API client is required');
  });

  test('should throw an error if discourseClient is not provided', () => {
    expect(() => new CourseCategoryMapper({ canvasClient: mockCanvasClient })).toThrow('Discourse API client is required');
  });

  test('getDiscourseCategory should return null if no mapping exists', async () => {
    db.query.mockResolvedValueOnce({ rows: [] });

    const result = await mapper.getDiscourseCategory(1);

    expect(result).toBeNull();
    expect(db.query).toHaveBeenCalledWith(
      'SELECT discourse_category_id FROM course_category_mappings WHERE canvas_course_id = $1',
      [1]
    );
  });

  test('getDiscourseCategory should return category if mapping exists', async () => {
    db.query.mockResolvedValueOnce({ rows: [{ discourse_category_id: 123 }] });
    mockDiscourseClient.getCategory.mockResolvedValueOnce({ id: 123, name: 'Test Category' });

    const result = await mapper.getDiscourseCategory(1);

    expect(result).toEqual({ id: 123, name: 'Test Category' });
    expect(mockDiscourseClient.getCategory).toHaveBeenCalledWith(123);
  });

  test('createDiscourseCategory should create a new category and store mapping', async () => {
    mockCanvasClient.getCourse.mockResolvedValueOnce({ id: 1, name: 'Test Course' });
    mockDiscourseClient.createCategory.mockResolvedValueOnce({ id: 456, name: 'Test Category' });
    db.query.mockResolvedValueOnce({ rows: [] }); // No existing mapping

    const result = await mapper.createDiscourseCategory(1);

    expect(result).toEqual({ id: 456, name: 'Test Category' });
    expect(mockCanvasClient.getCourse).toHaveBeenCalledWith(1);
    expect(mockDiscourseClient.createCategory).toHaveBeenCalledWith(
      expect.objectContaining({ name: 'Test Course' })
    );
    expect(db.query).toHaveBeenCalledWith(
      'INSERT INTO course_category_mappings (canvas_course_id, discourse_category_id) VALUES ($1, $2)',
      [1, 456]
    );
  });

  test('deleteMapping should delete mapping and optionally the category', async () => {
    db.query.mockResolvedValueOnce({ rows: [{ discourse_category_id: 789 }] });

    const result = await mapper.deleteMapping(1, true);

    expect(result).toBe(true);
    expect(mockDiscourseClient.deleteCategory).toHaveBeenCalledWith(789);
    expect(db.query).toHaveBeenCalledWith(
      'DELETE FROM course_category_mappings WHERE canvas_course_id = $1',
      [1]
    );
  });
});
