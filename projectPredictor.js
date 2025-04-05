// New file: c:\Users\Tim\Desktop\LMS\projectPredictor.js
class ProjectPredictor {
  constructor(metrics) {
    this.metrics = metrics;
  }
  
  /**
   * Predict project completion dates based on velocity
   */
  predictCompletion() {
    console.log("Predicting completion dates...");
    const predictions = this.metrics.predictions.estimates;
    const velocity = this.metrics.predictions.velocityData;
    const now = new Date();

    // Models
    const remainingModels = Math.max(0, this.metrics.models.total - this.metrics.models.implemented);
    const weeksForModels = velocity.models > 0 ? remainingModels / velocity.models : Infinity;
    predictions.models = {
        remaining: remainingModels,
        weeks: weeksForModels,
        date: isFinite(weeksForModels) ? this.addWeeks(now, weeksForModels).toISOString().split('T')[0] : 'N/A'
    };

    // API Endpoints
    const remainingApi = Math.max(0, this.metrics.apiEndpoints.total - this.metrics.apiEndpoints.implemented);
    const weeksForApi = velocity.apiEndpoints > 0 ? remainingApi / velocity.apiEndpoints : Infinity;
    predictions.apiEndpoints = {
        remaining: remainingApi,
        weeks: weeksForApi,
        date: isFinite(weeksForApi) ? this.addWeeks(now, weeksForApi).toISOString().split('T')[0] : 'N/A'
    };

    // UI Components
    const remainingUi = Math.max(0, this.metrics.uiComponents.total - this.metrics.uiComponents.implemented);
    const weeksForUi = velocity.uiComponents > 0 ? remainingUi / velocity.uiComponents : Infinity;
    predictions.uiComponents = {
        remaining: remainingUi,
        weeks: weeksForUi,
        date: isFinite(weeksForUi) ? this.addWeeks(now, weeksForUi).toISOString().split('T')[0] : 'N/A'
    };

    // Overall Project (simplistic - based on max weeks)
    const maxWeeks = Math.max(weeksForModels, weeksForApi, weeksForUi);
    predictions.project = {
        weeks: maxWeeks,
        date: isFinite(maxWeeks) ? this.addWeeks(now, maxWeeks).toISOString().split('T')[0] : 'N/A'
    };

    console.log(`Completion Predictions: Models=${predictions.models.date}, API=${predictions.apiEndpoints.date}, UI=${predictions.uiComponents.date}, Project=${predictions.project.date}`);
  }

  /**
   * Helper to add weeks to a date
   */
  addWeeks(date, weeks) {
    const result = new Date(date);
    result.setDate(result.getDate() + weeks * 7);
    return result;
  }
}

module.exports = ProjectPredictor;