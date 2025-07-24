import { useState } from 'react';
import { Link } from 'react-router-dom';
import { MoreHorizontal, Edit, History, Power, PowerOff } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { PersonaForm } from './persona-form';
import { makeRequest } from '@/lib/api';
import type { ProjectPersonaWithTemplate, ApiResponse } from 'shared/types';

interface PersonaCardProps {
  persona: ProjectPersonaWithTemplate;
  onUpdate: () => void;
}

export function PersonaCard({ persona, onUpdate }: PersonaCardProps) {
  const [showEditDialog, setShowEditDialog] = useState(false);
  const [updating, setUpdating] = useState(false);

  const getScoreColor = (score: number) => {
    if (score < 0) return 'bg-destructive';
    if (score < 10) return 'bg-muted';
    if (score < 25) return 'bg-blue-500';
    if (score < 100) return 'bg-green-500';
    return 'bg-yellow-500';
  };

  const getScoreLabel = (score: number, type: 'P' | 'Q') => {
    if (score < 0) return 'Issues';
    if (score < 10) return 'Standard';
    if (score < 25) return 'Senior';
    if (score < 100) return 'Elite';
    return type === 'P' ? 'Ultra Mega' : 'Master';
  };

  const handleToggleActive = async () => {
    setUpdating(true);
    try {
      const response = await makeRequest(
        `/api/personas/projects/${persona.project_id}/personas/${persona.id}`,
        {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ is_active: !persona.is_active }),
        }
      );
      
      const data: ApiResponse<any> = await response.json();
      if (data.success) {
        onUpdate();
      }
    } catch (err) {
      console.error('Failed to toggle persona status:', err);
    } finally {
      setUpdating(false);
    }
  };

  const displayName = persona.custom_name || persona.template_name;
  const roleType = persona.template_role_type.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());

  return (
    <>
      <Card className={`transition-all hover:shadow-md ${!persona.is_active ? 'opacity-60' : ''}`}>
        <CardHeader className="pb-3">
          <div className="flex items-start justify-between">
            <div className="flex-1 min-w-0">
              <CardTitle className="text-lg font-semibold truncate">
                {displayName}
              </CardTitle>
              <p className="text-sm text-muted-foreground mt-1">
                {roleType}
              </p>
            </div>
            <div className="flex items-center space-x-1">
              <Badge variant={persona.is_active ? 'default' : 'secondary'}>
                {persona.is_active ? 'Active' : 'Inactive'}
              </Badge>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="sm">
                    <MoreHorizontal className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => setShowEditDialog(true)}>
                    <Edit className="mr-2 h-4 w-4" />
                    Edit
                  </DropdownMenuItem>
                  <DropdownMenuItem asChild>
                    <Link to={`/team/${persona.id}`}>
                      <History className="mr-2 h-4 w-4" />
                      View History
                    </Link>
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem 
                    onClick={handleToggleActive}
                    disabled={updating}
                  >
                    {persona.is_active ? (
                      <>
                        <PowerOff className="mr-2 h-4 w-4" />
                        Deactivate
                      </>
                    ) : (
                      <>
                        <Power className="mr-2 h-4 w-4" />
                        Activate
                      </>
                    )}
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground line-clamp-2">
              {persona.template_description}
            </p>
            
            <div className="grid grid-cols-2 gap-2">
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-medium">Professionalism</span>
                  <span className="text-xs font-mono">
                    {Math.round(persona.professionalism_score * 10) / 10}
                  </span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="flex-1 h-2 bg-muted rounded-full overflow-hidden">
                    <div 
                      className={`h-full transition-all ${getScoreColor(persona.professionalism_score)}`}
                      style={{ 
                        width: `${Math.max(0, Math.min(100, (persona.professionalism_score / 25) * 100))}%` 
                      }}
                    />
                  </div>
                </div>
                <p className="text-xs text-muted-foreground">
                  {getScoreLabel(persona.professionalism_score, 'P')}
                </p>
              </div>

              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-medium">Quality</span>
                  <span className="text-xs font-mono">
                    {Math.round(persona.quality_score * 10) / 10}
                  </span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="flex-1 h-2 bg-muted rounded-full overflow-hidden">
                    <div 
                      className={`h-full transition-all ${getScoreColor(persona.quality_score)}`}
                      style={{ 
                        width: `${Math.max(0, Math.min(100, (persona.quality_score / 25) * 100))}%` 
                      }}
                    />
                  </div>
                </div>
                <p className="text-xs text-muted-foreground">
                  {getScoreLabel(persona.quality_score, 'Q')}
                </p>
              </div>
            </div>

            {persona.custom_instructions && (
              <div className="pt-2 border-t">
                <p className="text-xs font-medium mb-1">Custom Instructions</p>
                <p className="text-xs text-muted-foreground line-clamp-2">
                  {persona.custom_instructions}
                </p>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      <PersonaForm
        open={showEditDialog}
        onOpenChange={setShowEditDialog}
        projectId={persona.project_id}
        persona={persona}
        onSuccess={onUpdate}
      />
    </>
  );
}