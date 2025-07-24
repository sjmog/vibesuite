import { useState, useEffect } from 'react';
import { Calendar, TrendingUp, TrendingDown, Activity } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { makeRequest } from '@/lib/api';
import type { PersonaActivityWithTask, ApiResponse } from 'shared/types';

interface ActivityHistoryProps {
  personaId: string;
  projectId: string;
}

export function ActivityHistory({ personaId, projectId }: ActivityHistoryProps) {
  const [activities, setActivities] = useState<PersonaActivityWithTask[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchActivities = async () => {
      setLoading(true);
      try {
        const response = await makeRequest(
          `/api/personas/projects/${projectId}/personas/${personaId}/activities?limit=50`
        );
        const data: ApiResponse<PersonaActivityWithTask[]> = await response.json();
        
        if (data.success && data.data) {
          setActivities(data.data);
        } else {
          setError(data.message || 'Failed to load activities');
        }
      } catch (err) {
        setError('Failed to load activities');
      } finally {
        setLoading(false);
      }
    };

    fetchActivities();
  }, [personaId, projectId]);

  const getActivityIcon = (activityType: string) => {
    switch (activityType) {
      case 'task_assigned':
      case 'task_completed':
      case 'task_failed':
        return <Activity className="h-4 w-4" />;
      case 'kudos_received':
        return <TrendingUp className="h-4 w-4 text-green-600" />;
      case 'wtf_received':
      case 'process_violation':
      case 'quality_issue':
        return <TrendingDown className="h-4 w-4 text-red-600" />;
      default:
        return <Calendar className="h-4 w-4" />;
    }
  };

  const getActivityColor = (activityType: string) => {
    switch (activityType) {
      case 'task_completed':
      case 'kudos_received':
        return 'bg-green-100 text-green-800';
      case 'task_failed':
      case 'wtf_received':
      case 'process_violation':
      case 'quality_issue':
        return 'bg-red-100 text-red-800';
      case 'task_assigned':
        return 'bg-blue-100 text-blue-800';
      case 'imported':
        return 'bg-purple-100 text-purple-800';
      case 'score_adjustment':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const formatActivityType = (activityType: string) => {
    return activityType.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  };

  const formatScoreChange = (profChange: number, qualChange: number) => {
    const changes = [];
    if (profChange !== 0) {
      changes.push(`P: ${profChange > 0 ? '+' : ''}${profChange}`);
    }
    if (qualChange !== 0) {
      changes.push(`Q: ${qualChange > 0 ? '+' : ''}${qualChange}`);
    }
    return changes.join(', ');
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-center">
          <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary mx-auto"></div>
          <p className="mt-2 text-sm text-muted-foreground">Loading activities...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-8">
        <p className="text-destructive">{error}</p>
      </div>
    );
  }

  if (activities.length === 0) {
    return (
      <div className="text-center py-8">
        <div className="mx-auto w-12 h-12 bg-muted rounded-full flex items-center justify-center mb-4">
          <Activity className="h-6 w-6 text-muted-foreground" />
        </div>
        <h3 className="text-lg font-semibold mb-2">No activities yet</h3>
        <p className="text-muted-foreground">
          Activity history will appear here once the persona starts participating in tasks.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-lg font-semibold mb-2">Recent Activity</h3>
        <p className="text-sm text-muted-foreground">
          Showing the last {activities.length} activities for this persona
        </p>
      </div>

      <div className="space-y-3">
        {activities.map((activity) => (
          <Card key={activity.id} className="p-4">
            <div className="flex items-start space-x-3">
              <div className="flex-shrink-0 mt-1">
                {getActivityIcon(activity.activity_type)}
              </div>
              
              <div className="flex-1 min-w-0">
                <div className="flex items-center space-x-2 mb-1">
                  <Badge 
                    variant="secondary" 
                    className={getActivityColor(activity.activity_type)}
                  >
                    {formatActivityType(activity.activity_type)}
                  </Badge>
                  
                  {activity.task_title && (
                    <Badge variant="outline" className="text-xs">
                      {activity.task_title}
                    </Badge>
                  )}
                  
                  <Badge variant="outline" className="text-xs">
                    {activity.task_size}
                  </Badge>
                </div>
                
                <p className="text-sm text-foreground mb-2">
                  {activity.description}
                </p>
                
                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <span>
                    {new Date(activity.created_at).toLocaleDateString()} at{' '}
                    {new Date(activity.created_at).toLocaleTimeString()}
                  </span>
                  
                  {(activity.professionalism_change !== 0 || activity.quality_change !== 0) && (
                    <span className="font-mono">
                      {formatScoreChange(activity.professionalism_change, activity.quality_change)}
                    </span>
                  )}
                </div>
                
                {activity.metadata && (
                  <details className="mt-2">
                    <summary className="text-xs text-muted-foreground cursor-pointer hover:text-foreground">
                      View metadata
                    </summary>
                    <pre className="text-xs bg-muted p-2 rounded mt-1 overflow-x-auto">
                      {JSON.stringify(JSON.parse(activity.metadata), null, 2)}
                    </pre>
                  </details>
                )}
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}