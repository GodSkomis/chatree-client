import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SidebarChat } from './sidebar-chat';

describe('SidebarChat', () => {
  let component: SidebarChat;
  let fixture: ComponentFixture<SidebarChat>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SidebarChat]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SidebarChat);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
