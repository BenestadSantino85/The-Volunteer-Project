import { VolunteerTime, CommunityServiceInitiative } from 'volunteer-time-api';

class CommunityServiceInitiative {
  private volunteerTime: VolunteerTime;
 
  constructor() {
   this.volunteerTime = new VolunteerTime();
  }
  
  //Method to promote the Community Service Initiative
  public promoteInitiative(): void {
   this.volunteerTime.announceInitiative();
	 this.volunteerTime.advocateInitiative();
	 this.volunteerTime.hostEvents();
	 this.volunteerTime.raiseAwareness();
  }
 
  //Method to recruit volunteers for Community Service 
  public recruitVolunteers(): void {
   this.volunteerTime.createVolunteerProfileForm();
	 this.volunteerTime.reachOutToVolunteers();
	 this.volunteerTime.hostRecruitmentEvents();
	 this.volunteerTime.sendApplicationReminders();
  }
 
  //Method to manage the volunteers
  public manageVolunteers(): void {
   this.volunteerTime.createVolunteerDatabase();
	 this.volunteerTime.maintainVolunteerHours();
	 this.volunteerTime.provideOrientationSessions();
	 this.volunteerTime.provideFeedbackPrograms();
  }
  
  //Method to track the impact of the initiative
  public trackImpact(): void {
   this.volunteerTime.captureVolunteerActivity();
	 this.volunteerTime.analyzeVolunteerData();
	 this.volunteerTime.conductImpactSurveys();
	 this.volunteerTime.publishImpactReports();
  }
}