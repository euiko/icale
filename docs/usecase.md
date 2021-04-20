# Overview

Before detailing the use cases we need to talks about the problem we try to address with this litle project.
As my experience I'm the one of the people that are like trying new things, but sometimes in many cases I forgot what I want to do because too many things are on my wishlist.
To overcome this problem I think it is a good idea to make a schedules application that includes some form of wishlist to be done.
So we can prioritize what our goals next from our wishlist.
It is also nice to have some kind of gamification, e.g. like achievement badge and leveling to maintain personal goals.
For this purpose maybe good to have this project opensource :D.

# Use Cases

## User
- Of course need user related mechanism to login/register :D

## Character profile
- The user/personnel are represented with character so there is some kind of character creation that you can choose to represent your avatar.
- There is some set of classes to be choosen
- There is also experience system, with leveling capabilities.
- (Optional) It is also nice to have an skill tree system, so we can allocate some skill point earned to learn skill

## Tags / Category
- It is are useful to be able to group any form of takss e.g. to do, schedules, milestone, etc.
- It can be for the dashboard overview about what kind of activity that the personnel has been done.

## Achievement
- Useful to maintain the personnal motivation, so s/he have somekind of personal achievement. e.g. Cleaner of my self (do cleaning schedules for 10 times consecutively)
- It can be associated with tags, to do list, schedules, or milestone.
- We can associate experience point on completion

## To Do List
- Need basic mechanism to maintain all the tasks in form of to do list.
- Created to do could be associated with specific time or date, or when the case can be not attached into any time
- Could have a deadline associated with to do.
- We need to be able associate to do with some kind of goals/milestone so we can also track its progress.
- Also need to be able to associate it with milestone
- We can associate each to do with experience point to be earned when completing to do.
- We can attach achievement to gain on completion
- Any state changes are being kept as a logs

## Schedules
- Any recurring tasks are represented as schedule so we can maintain and create to do list based on this schedules
- Any tasks defined here most of them are like the one on the to do list, but without the exact time to do the thing
- We can also associate each tasks with experience point such as in to do.
- We can add achievement to gain on some kind of rules.
- We can close/stop the schedule when it is not needed anymore
- Any state changes associated with schedules are also logged here, e.g. tasks completion, etc.

## Milestone / Goals
- This special kind of schedule are can be arranged to contain several tasks and you can track the goal/milestone achievement here.
- Any completed tasks here are affect the milestone progress.
- Milestone are often associated with deadline, but its must not be.
- We can predetermine progression, e.g. initial, doing, reviewinng. Useful for personal projects.
- We can add achievement at any point of this milestone progress, e.g. in tasks, progression, completion, etc.
- We can close/stop the schedule when it is not needed anymore
- Any state change assoicated with milestones/goals are also logged here, e.g. tasks completion, progression achieved, achievement achieved, etc.

## UI/X
- All to do menu or tasks are categorized based on its tags/categories, the user need to be able to choose to pin the tags/catogires in the menu.
- There is one button to create any form of tasks to do, schedule, milestone
- There is shortcut to create to do directly.
- There is distinct page to be able to view all the to do regardless its tags.
- Still have distinct pages to manage schedule and milestone/goal, for easier maintenance.
- There is dashboard to view your character activity history and look for insight of your accomplisment